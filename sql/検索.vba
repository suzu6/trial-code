Option Explicit
Sub ★逆展開_検索対象コードを選択()
Application.ScreenUpdating = False

    Dim ws逆展開 As Worksheet
    Set ws逆展開 = Worksheets(STRシート名_逆展開)
    Form品目検索.Show      '検索フォームを表示します。
    ws逆展開.Cells(逆展開_検索対象_入力_ROW, 逆展開_検索対象_コード_COL).Value = STR_FORM品目検索_検索結果_品目コード
    
    
    '選択セルが使用品目範囲に被ると、うまく表示されないっぽいので、被らないセルを選択する
    ws逆展開.Cells(逆展開_検索対象_入力_ROW, 逆展開_検索対象_コード_COL).Select

    
    '前回の展開データがある場合は消去
    If Cells(逆展開_展開先_先頭_ROW, 1).Value <> "" Then
        Dim 最終_row As Long
        最終_row = Cells(Rows.Count, 1).End(xlUp).Row
        Range(Cells(逆展開_展開先_先頭_ROW, 1), Cells(最終_row, 逆展開_展開先_子_項目2_COL)).Value = ""
    Else
    End If
    
    '検索するデータを配列に格納する。
    Dim ws品目マスタ As Worksheet
    Set ws品目マスタ = Worksheets(STRシート名_品目マスタ)

    With ws品目マスタ
        Dim var配列_品目マスタ As Variant
        Dim 品目マスタ_last_row As Long
        品目マスタ_last_row = .Cells(Rows.Count, 1).End(xlUp).Row
        var配列_品目マスタ = .Range(.Cells(2, 1), .Cells(品目マスタ_last_row, 品目マスタ_LAST_COL)).Value
    End With

    '逆展開の検索対象コードを検索（その前に検索フォームでコードを探してる）
    Dim strコード番号 As String
    Dim i As Long
        
    strコード番号 = STR_FORM品目検索_検索結果_品目コード
    If strコード番号 = "" Then   '検索対象コードが空欄でないなら、配列から探す
      End
    End If
    
    
    For i = LBound(var配列_品目マスタ) To UBound(var配列_品目マスタ)      'ループ処理で配列を検索し、行数を変数へ格納
      If var配列_品目マスタ(i, 品目マスタ_コード_COL) <> strコード番号 Then  '条件に一致するコードが見つかったら
        continue
      End If

      '計算表のセルに、栄養成分一覧の値をセットし、ループ処理iから抜けてjのループへ
      With ws逆展開
          .Cells(逆展開_検索対象_入力_ROW, 逆展開_品目名_COL).Value = var配列_品目マスタ(i, 品目マスタ_品名_COL)
          .Cells(逆展開_検索対象_入力_ROW, 逆展開_大分類_COL).Value = var配列_品目マスタ(i, 品目マスタ_大分類_COL)
          .Cells(逆展開_検索対象_入力_ROW, 逆展開_中分類_COL).Value = var配列_品目マスタ(i, 品目マスタ_中分類_COL)
          .Cells(逆展開_検索対象_入力_ROW, 逆展開_小分類_COL).Value = var配列_品目マスタ(i, 品目マスタ_小分類_COL)
          .Cells(逆展開_検索対象_入力_ROW, 逆展開_使用停止フラグ_COL).Value = var配列_品目マスタ(i, 品目マスタ_使用停止フラグ_COL)
      End With
      Exit For    '該当コードが見つかったら、書き込んで検索終了(ループから抜ける)
    Next i

    Call ★逆展開

Application.ScreenUpdating = True
End Sub
