#[derive(Clone, Debug, PartialEq)]
struct Item { next: Option<Box<Item>>, value: i32 }

impl Item {
 fn create(list: Vec<i32>) -> Self {
  let mut item: Item = Item { next: None, value: 0 };

  for (index, value) in list.iter().rev().enumerate() {
   if index == 0 {
    item.next = None;

   } else {//if index == 0 {
    item.next = Some(Box::new(Item { next: item.next, value: item.value }));

   }//} else {//if index == 0 {

   item.value = *value;
  }//for (index, value) in list.iter().enumerate() {

  item
 }//fn create(list: Vec<i32>) -> Self {

 fn delete( &mut self, inverse: usize ) -> bool {
  if inverse == 0 {
   false

  } else {//if inverse == 0 {
   let length: usize = self.length();

   if inverse > length {
    false

   } else {//if inverse > length {
    if length < 2 {
     false

    } else {//if length < 2 {
     if length == 2 {
      if inverse == 2 {
       let last: &Item = self.last();

       self.value = last.value;
      }//if inverse == 2 {

      self.next = None;

      true

     } else {//if length == 2 {
      if inverse == 1 {
       if let Some(ref mut penultimate) = self.penultimate() {
        penultimate.next = None;

        true

       } else {//if let Some(ref mut penultimate) = self.penultimate() {
        false

       }//} else {//if let Some(ref mut penultimate) = self.penultimate() {

      } else if inverse == length {//if inverse == 1 {
       if let Some(ref mut self_next) = self.next.take() {
        self.next = self_next.next.take();

        self.value = self_next.value;

        true

       } else {//if let Some(ref mut self_next) = self.next {
        false

       }//} else {//if let Some(ref mut self_next) = self.next {

      } else {//} else if inverse == length {//if inverse == 1 {
       let before: usize = length - inverse;

       if let Some(ref mut previous) = self.item(before) {
        if let Some(ref mut current) = previous.next.take() {
         previous.next = current.next.take();

         true

        } else {//if let Some(ref mut current) = previous.next.take() {
         false

        }//} else {//if let Some(ref mut current) = previous.next.take() {

       } else {//if let Some(ref mut previous) = self.item(order - 1) {
        false

       }//} else {//if let Some(ref mut previous) = self.item(order - 1) {
      }//} else {//} else if inverse == length {//if inverse == 1 {
     }//} else {//if length == 2 {
    }//} else {//if length < 2 {
   }//} else {//if inverse > length {
  }//} else {//if inverse == 0 {
 }//fn delete( &mut self, inverse: usize ) -> bool {

 fn item(&mut self, order: usize) -> Option<&mut Item> {
  match order {
   0 => { None       }
   1 => { Some(self) }
   _ => {
    match self.next {
     Some(ref mut self_next) => { self_next.item(order - 1) }
     None                    => { None                      }
    }//match self.next {
   }//_ => {
  }//match order {
 }//fn item(&mut self, order: usize) -> Option<&mut Item> {

 fn last(&mut self) -> &Item {
  match self.next {
   Some(ref mut self_next) => { self_next.last() }
   None                    => { self             }
  }//match self.next {
 }//fn last(&mut self) -> &Item {
 
 fn length(&mut self) -> usize {
  match self.next {
   Some(ref mut self_next) => { 1 + self_next.length() }
   None                    => { 1                      }
  }//match self.next {
 }//fn length(&mut self) -> usize {

 fn penultimate(&mut self) -> Option<&mut Item> {
  if let Some(ref mut self_next) = self.next {
   if self_next.next.is_none() {
    Some(self)

   } else {//if self_next.next.is_none() {
    self.next.as_mut().expect("Exists").penultimate()

   }//} else {//if self_next.next.is_none() {

  } else {//if let Some(ref mut self_next) = self.next {
   None

  }//} else {//if let Some(ref mut self_next) = self.next {
 }//fn penultimate(&mut self) -> Option<&mut Item> {
}//impl List {

fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn main() {
 'vector: loop {
  println!("\r\n\r\nvector:");

  let mut input: String = request();
 
  if &input[..] == "exit" {
   break;   

  } else {//if &input[..] == "exit" {
   match serde_json::from_str::<Vec<i32>>(&input[..]) {
    Ok(list) => {
     let size: usize = list.len();

     if size > 1 {
      let mut item: Item = Item::create(list);

      println!("item: {:?}", item);

      loop {
       println!("\r\ninverse:");

       input = request();

       match &input[..] {
        "back" => { break        ; }
        "exit" => { break 'vector; }
        _ => {
         match (&input[..]).trim().parse::<usize>() {
          Ok(inverse) => {
           println!("item.delete(inverse): {:?}", item.delete(inverse));

           println!("item: {:?}", item);
          }//Ok(inverse) => {

          Err(error) => {
           println!("Error: {:?}", error);

          }//Err(error) => {
         }//match (&input[..]).trim().parse::<usize>() {
        }//_ => {
       }//match &input[..] {
      }//loop {
     }//if size > 1 {
    }//Ok(list) => {

    Err(error) => {
     println!("Error: {:?}", error);

    }//Err(error) => {
   }//match serde_json::from_str::<Vec<i32>>(&input[..]) {
  }//} else {//if &input[..] == "exit" {
 }//'vector: loop {
}//fn main() {
