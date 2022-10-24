struct Solution;


// https://leetcode.com/contest/weekly-contest-316/problems/determine-if-two-events-have-conflict/
impl Solution {
    pub fn parse_time(s: String) -> i32 {
        return s[..2].parse::<i32>().unwrap() * 60 + s[3..].parse::<i32>().unwrap();
    }
    
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
       
        // first parse the string to number, which can be compared
        
        let ref rs1 = &event1[0];
        let s1 = Solution::parse_time(rs1.to_string());
        
        let ref re1 = &event1[1];
        let e1 = Solution::parse_time(re1.to_string());
        
        let ref rs2 = &event2[0];
        let s2 = Solution::parse_time(rs2.to_string());
        
        let ref re2 = &event2[1];
        let e2 = Solution::parse_time(re2.to_string());
        
        // print!("{:?}", s1);
        // second apply the rule to check overlap
        // if e2 < s1 or e1 < s2, there is no overlap, i.e.
        // if e2 >= s1 and e1 >= s2, there is overlap
        return e2 >= s1 && e1 >= s2
    }    
}

fn main() {

    // use macro for output;
    println!("Hello, world!");

    // Two ways of creating vector;
    let event1 = vec!["01:15".to_string(),"02:00".to_string()];
    let mut event2 = Vec::new();
    event2.push("02:00".to_string());
    event2.push("03:00".to_string());
    println!("{:?}", Solution::have_conflict(event1, event2));
}
