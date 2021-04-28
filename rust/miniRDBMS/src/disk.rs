use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;
/// ディスクマネージャ
/// https://github.com/KOBA789/relly/blob/wdb/src/disk.rs
///
/// io::Resultはio系のエラーで中身が成功したときの返り値
/// Selfは自分の型と()はviod
/// &mut selfがないメソッドはスタティックメソッド
///
/// p78ではdata_file_pathだが、p79からのコードはheap_file_pathになっている（文中はdata_file_path）。
use std::{
  convert::TryInto,
  fs::{File, OpenOptions},
};

use zerocopy::{AsBytes, FromBytes};

pub const PAGE_SIZE: usize = 4096;

/// ページIDを表す型
/// 足し算など無意味な演算をエラーにしておく
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromBytes, AsBytes)]
#[repr(C)]
pub struct PageId(pub u64);
impl PageId {
  pub const INVALID_PAGE_ID: PageId = PageId(u64::MAX);

  pub fn valid(self) -> Option<PageId> {
      if self == Self::INVALID_PAGE_ID {
          None
      } else {
          Some(self)
      }
  }

  pub fn to_u64(self) -> u64 {
      self.0
  }
}

pub struct DiskManager {
  // ヒープファイルのファイルディスクリプタ
  heap_file: File,
  // 採番するページIDを決めるカウンタ
  next_page_id: u64,
}

impl Default for PageId {
  fn default() -> Self {
    Self::INVALID_PAGE_ID
  }
}

impl From<Option<PageId>> for PageId {
  fn from(page_id: Option<PageId>) -> Self {
    page_id.unwrap_or_default()
  }
}

impl From<&[u8]> for PageId {
  fn from(bytes: &[u8]) -> Self {
    let arr = bytes.try_into().unwrap();
    PageId(u64::from_ne_bytes(arr))
  }
}

impl DiskManager {
  // コンストラクタ
  pub fn new(heap_file: File) -> io::Result<Self> {
    // ファイルサイズを取得
    let heap_file_size = heap_file.metadata()?.len();
    let next_page_id = heap_file_size / PAGE_SIZE as u64;
    Ok(Self {
      heap_file,
      next_page_id,
    })
  }

  // ファイルパスを指定して開く
  // pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {
  pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
    let heap_file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(heap_file_path)?; // I/Oエラーが起きるかもしれいない。エラーの場合は早期リターン
    Self::new(heap_file)
  }

  // 新しいページIDを採番する
  pub fn allocate_page(&mut self) -> PageId {
    let page_id = self.next_page_id;
    self.next_page_id += 1; // 採番する度にインクリメントする
    PageId(page_id)
  }

  // ページのデータを読みだす
  pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
    // オフセットを計算
    let offset = PAGE_SIZE as u64 * page_id.to_u64();
    // ページ先頭へシーク
    self.heap_file.seek(SeekFrom::Start(offset))?;
    // データを書きこむ
    self.heap_file.read_exact(data)
  }

  // データをページに書き出す
  pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
    // オフセットを計算
    let offset = PAGE_SIZE as u64 * page_id.to_u64();
    // ページ先頭へシーク
    self.heap_file.seek(SeekFrom::Start(offset))?;
    // データを書きこむ
    self.heap_file.write_all(data)
  }

  pub fn sync(&mut self) -> io::Result<()> {
    self.heap_file.flush()?;
    self.heap_file.sync_all()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::NamedTempFile;

  #[test]
  fn test_disk() {
    let (data_file, data_file_path) = NamedTempFile::new().unwrap().into_parts();
    let mut disk = DiskManager::new(data_file).unwrap();
    let mut hello = Vec::with_capacity(PAGE_SIZE);
    hello.extend_from_slice(b"hello");
    hello.resize(PAGE_SIZE, 0);
    let hello_page_id = disk.allocate_page();
    disk.write_page_data(hello_page_id, &hello).unwrap();
    let mut world = Vec::with_capacity(PAGE_SIZE);
    world.extend_from_slice(b"world");
    world.resize(PAGE_SIZE, 0);
    let world_page_id = disk.allocate_page();
    disk.write_page_data(world_page_id, &world).unwrap();
    drop(disk);
    let mut disk2 = DiskManager::open(&data_file_path).unwrap();
    let mut buf = vec![0; PAGE_SIZE];
    disk2.read_page_data(hello_page_id, &mut buf).unwrap();
    assert_eq!(hello, buf);
    disk2.read_page_data(world_page_id, &mut buf).unwrap();
    assert_eq!(world, buf);
  }

  // #[test]
  // fn test_positive() {
  //   assert!(true);
  // }
  // #[test]
  // fn test_negative() {
  //   assert!(false);
  // }
}
