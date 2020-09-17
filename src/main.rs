fn main() {

    let vec:Vec<i32>=vec![-1,-2,-3,4,5,-6,-7,-8];
    let val=max_sub_array(vec);
    println!("{}",val);
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    return sub_array(&nums)
}
    
pub fn sub_array(nums: &[i32]) -> i32 {
    if nums.len()==1{
        if nums[0]>0{
            return nums[0]
        } else{
            return 0
        }
    }
    if nums.len()==0{
        return 0
    }
    
    let mid=nums.len()/2;

    
    let left_max=sub_array(&nums[..mid]);
    let right_max=sub_array(&nums[mid..]);
    let mid_max=max_subarray_cross_mid(nums,mid);
    
    return std::cmp::max(std::cmp::max(left_max,right_max),mid_max)
}

pub fn max_subarray_cross_mid(nums: &[i32],mid:usize)->i32{
    let mut left_max=nums[mid-1];
    let mut sum=nums[mid-1];
    
    let mut i:i32=mid as i32-2;
    while i >= 0 {
        sum += nums[i as usize];
        i-=1;
        if sum > left_max {
          left_max = sum;
        }
    }
    
    let mut right_max=nums[mid];
    sum=nums[mid];
    i = mid as i32+ 1;
    while i < nums.len() as i32 {
        sum += nums[i as usize];
        i+=1;
        if sum > right_max {
          right_max = sum;
        }
    }
    
    return left_max+right_max
}