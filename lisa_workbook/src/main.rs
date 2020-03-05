use text_io::*;

fn pages_per_chapter(k: usize, n: usize) -> usize {
    let result = n / k;
    if n % k == 0 {
        result
    } else {
        result + 1
    }
}

fn solution(k: usize, chapters: Vec<usize>) -> usize {
    let mut chapter_page = 1;
    let mut result = 0;
    
    for problems in chapters {
        let pages_per_chapter = pages_per_chapter(k, problems);
        for page_offset in 0..pages_per_chapter {
            let problems_from_chapter_start = k * page_offset;
            let last_problem_on_page = problems_from_chapter_start + k;
            let current_page = chapter_page + page_offset;
            if problems_from_chapter_start < current_page 
                && current_page <= last_problem_on_page 
                && current_page <= problems {
                result += 1;
            }
        }

        chapter_page += pages_per_chapter;
    }

    result
}

fn main() {
    let n = read!();
    let k = read!();
    let input = (0..n).map(|_| read!()).collect();
    let result = solution(k, input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pages_per_chapter() {
        assert_eq!(pages_per_chapter(3, 5), 2);
        assert_eq!(pages_per_chapter(3, 6), 2);
    }

    #[test]
    fn test_single() {
        assert_eq!(solution(1, vec![1, 1, 1, 1]), 1);
    }

    #[test]
    fn test_sample_input() {
        assert_eq!(solution(3, vec![4, 2]), 1);
        assert_eq!(solution(3, vec![4, 2, 6, 1, 10]), 4);
    }
}
