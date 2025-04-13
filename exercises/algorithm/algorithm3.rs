/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord>(array: &mut [T]){
	//TODO
    quick_sort(array)
}

fn quick_sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        // 基本情况: 空数组或单元素数组已排序
        return;
    }
    let pivot = partition(array); // 获取基准位置
    quick_sort(&mut array[..pivot]); // 递归排序左半部分
    quick_sort(&mut array[pivot + 1..]); // 递归排序右半部分
}

fn partition<T: Ord>(array: &mut [T]) -> usize {
    let pivot = array.len() - 1; // 选择最后一个元素作为基准
    let mut i = 0; // i 是小于基准的元素的边界, 为了避免每次都移动基准
    for j in 0..pivot {
        // 遍历除基准外的所有元素
        if array[j] <= array[pivot] {
            // 当前元素小于等于基准
            array.swap(i, j); // 把它放到i的位置
            i += 1; // 移动i边界
        }
    }
    array.swap(i, pivot); // 把基准放到正确位置
    i // 返回基准的最终位置
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}