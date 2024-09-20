pub fn load_csv<T: serde::de::DeserializeOwned, P: AsRef<std::path::Path>, F>(path: P, f: F)
where
  F: FnMut(T),
{
  csv::Reader::from_reader(std::fs::File::open(path).unwrap()).deserialize::<T>().map(|it| it.unwrap()).for_each(f);
}
