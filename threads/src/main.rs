use std::collections::BinaryHeap;
use std::io::BufReader;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    blame_todo("sdfsdf")?;
    Ok(())
}

// TODO 的可能格式为
// TODO(xxx): xxx
// //todo(xxx): xxx
// //FIXME(xxx): XXX
// /fixme(xxx): XxX
// TODO 可能单独出现在一行，也有可能出现在行未的注释中

// -  实现一个 blame_todo 函数，统计代码仓库中留下 TODO 最多的 5 个人的名字，按 TODO 数量降序排列，TODO 数量相同的按名字的字母序排列；

const TO_DO: &str = "TODO";

fn blame_todo(path: &str) -> io::Result<Vec<(String, usize)>> {
    let mut files = Vec::new();
    get_files(path, &mut files)?;

    let mut stats = HashMap::new();
    for f in files.iter() {
        stats = get_todo_stat(f)?;
    }

    Ok(get_top_n(stats, 5))
}

#[derive(PartialEq, Eq, Debug)]
struct Pair(String, usize);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

fn get_top_n(dic: HashMap<String, i32>, n: usize) -> Vec<(String, usize)> {
    let mut heap = BinaryHeap::new();
    for (k, v) in dic {
        heap.push(Pair(k, v as usize));
    }

    let mut ret = Vec::new();
    for _i in 0..n {
        let p = heap.pop().unwrap();
        ret.push((p.0, p.1));
    }

    ret
}

fn get_todo_stat(file: &File) -> io::Result<HashMap<String, i32>> {
    let mut ret = HashMap::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line?;
        if l.starts_with("//") && l.contains(TO_DO) {
            let v: Vec<_> = l.split(|c| c == '(' || c == ')').collect();
            ret.entry(v[1].to_owned())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    Ok(ret)
}

fn get_files(path: &str, files: &mut Vec<File>) -> io::Result<()> {
    let entries = fs::read_dir(path)?;
    for e in entries {
        let e = e?;
        let path = e.path();
        if path.is_dir() {
            get_files(path.to_str().unwrap(), files)?;
        } else {
            files.push(File::open(path)?);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        fs::{self, File},
        io::Write,
    };
    use tempfile::{tempdir, TempDir};
    use walkdir::WalkDir;

    use crate::{get_files, get_todo_stat};

    fn create_test_files() -> TempDir {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        let mut file = File::create(file_path).unwrap();

        fs::create_dir(dir.path().join("test1")).unwrap();
        let file_path1 = dir.path().join("test1/test1.rs");
        let mut file1 = File::create(file_path1).unwrap();

        writeln!(file, "//TODO(a): test file").unwrap();
        writeln!(file, "//TODO(b): test file").unwrap();
        writeln!(file, "//TODO(b): test file").unwrap();

        writeln!(file1, "//TODO(c): test file").unwrap();

        println!("{:#?}", WalkDir::new(&dir).into_iter().collect::<Vec<_>>());

        dir
    }

    #[test]
    fn test_get_files() {
        // Create a directory inside of `std::env::temp_dir()`.
        let dir = create_test_files();
        let mut v = Vec::new();
        let p = dir.path().to_str().unwrap();
        get_files(p, &mut v).unwrap();
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_get_todo_stat() {
        let dir = create_test_files();
        let mut v = Vec::new();
        let p = dir.path().to_str().unwrap();
        get_files(p, &mut v).unwrap();
        assert_eq!(
            get_todo_stat(&v[0]).unwrap(),
            HashMap::from([("a".to_owned(), 1), ("b".to_owned(), 2)])
        );
        assert_eq!(
            get_todo_stat(&v[1]).unwrap(),
            HashMap::from([("c".to_owned(), 1)])
        );
    }

    #[test]
    fn test_get_top_n() {
        let dic = HashMap::from([
            ("a".to_owned(), 1),
            ("b".to_owned(), 2),
            ("c".to_owned(), 3),
        ]);
        let ret = get_top_n(dic, 2);
        assert_eq!(ret, vec![("c".to_owned(), 3), ("b".to_owned(), 2),]);
    }
}
