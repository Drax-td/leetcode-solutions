struct ProductOfNumbers {
    prefix_products: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        ProductOfNumbers { prefix_products: vec![1] }
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix_products.clear();
            self.prefix_products.push(1);
        } else {
            if let Some(last) = self.prefix_products.last() {
                self.prefix_products.push(num * last);
            }
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        
        if k >= self.prefix_products.len() {
            return 0;
        }

        let last_index = self.prefix_products.len() - 1;
        self.prefix_products[last_index] / self.prefix_products[last_index - k]
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */