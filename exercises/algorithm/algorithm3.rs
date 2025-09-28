/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]) {
    let n = array.len();
    // 插入排序：从第二个元素开始（索引 i=1）
    for i in 1..n {
        let mut j = i;
        // 将 array[i] 插入到已排序序列 array[0..i-1] 的正确位置
        // 循环条件：j 必须大于 0，并且前一个元素 array[j-1] 大于当前元素 array[j]
        while j > 0 && array[j - 1] > array[j] {
            // 如果前一个元素更大，就交换它们
            array.swap(j - 1, j);
            // 继续向左检查
            j -= 1;
        }
    }
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
