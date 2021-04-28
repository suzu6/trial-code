/// ディスクマネージャ
/// https://github.com/KOBA789/relly/blob/wdb/src/disk.rs
///
/// io::Resultはio系のエラーで中身が成功したときの返り値
/// Selfは自分の型と()はviod
/// &mut selfがないメソッドはスタティックメソッド
/// 
/// // p78ではdata_file_pathだが、p79からのコードはheap_file_pathになっている（文中はdata_file_path）。

pub struct DiskManager {
  // ヒープファイルのファイルディスクリプタ
  heap_file: File,
  // 採番するページIDを決めるカウンタ
  next_page_id: u64,
}

impl DiskManager {
  // コンストラクタ
  pub fn new(heap_file: File) -> io::Result<Self> {
    // ファイルサイズを取得
    let heap_file_size = heap_file.metadata()?.len();
    let next_page_id = heap_file_size / PAGE_SIZE as u64;
    Ok(Self {
      heap_file,
      nextpage_id,
    })
  }

  // ファイルパスを指定して開く
  // pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> { 
  pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
    let heap_file = OpenOptions::new()
      .read(true)
      .write(true)
      .creat(true)
      .open(heap_file_path)?; // I/Oエラーが起きるかもしれいない。エラーの場合は早期リターン
    Self::new(heap_file)
  }

  // 新しいページIDを採番する
  pub fn allocate_page(&mut self) -> PageId {
    let page_id  =self.next_page_id;
    self.next_page_id += 1; // 採番する度にインクリメントする
    PageId(page_id)
  }

  // ページのデータを読みだす
  pub fn read_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
    // オフセットを計算
    let offset = PAGE_SIZE as u64 * page_id.to_u64();
    // ページ先頭へシーク
    selt.heap_file.seek(SeekFrom::Start(offset))?;
    // データを書きこむ
    self.heap_file.read_exact(data)
  }

  // データをページに書き出す
  pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
    // オフセットを計算
    let offset = PAGE_SIZE as u64 * page_id.to_u64();
    // ページ先頭へシーク
    selt.heap_file.seek(SeekFrom::Start(offset))?;
    // データを書きこむ
    self.heap_file.write_all(data)
  }
}

/// ページIDを表す型
/// 足し算など無意味な演算をエラーにしておく
pub struct  PageId(pub u64);