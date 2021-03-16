Option Explicit
Sub ★逆展開()
Application.ScreenUpdating = False
    
    Dim ws逆展開 As Worksheet
    Set ws逆展開 = Worksheets(STRシート名_逆展開)

    '検索対象コードが空欄なら、検索しない
    If ws逆展開.Cells(逆展開_検索対象_入力_ROW, 逆展開_検索対象_コード_COL).Value = "" Then
        MsgBox "検索対象の品目コードを指定してください"
        ws逆展開.Cells(逆展開_検索対象_入力_ROW, 逆展開_検索対象_コード_COL).Select
    Else

    'マクロ実行時間の計測
    Dim StartTime As Variant
    Dim StopTime As Variant
    'ここから実行時間のカウントを開始します
    StartTime = Time
    
    '処理に時間かかるので、メッセージを出す
    Form作成中.Show vbModeless
    Form作成中.Repaint
            
    'ここから呼び出し処理
    Call 逆展開_検索対象コードが構成マスタに登録されているか確認
    Call 逆展開_前回の検索結果が残っていれば削除
    Call 逆展開_1階層目の呼び出し
    Call 逆展開_検索対象の2階層以降の情報を抽出
    Call 逆展開_品目コード一括検索
                
    ws逆展開.Cells(逆展開_検索対象_入力_ROW, 逆展開_検索日時_COL).Value = Now
                
    Unload Form作成中
    StopTime = Time
    StopTime = StopTime - StartTime
    MsgBox "処理が完了しました。所要時間：" & Minute(StopTime) & "分" & _
                                              Second(StopTime) & "秒"

    End If
Application.ScreenUpdating = True
End Sub


Sub 逆展開_検索対象コードが構成マスタに登録されているか確認()
Application.ScreenUpdating = False
    Dim ws逆展開 As Worksheet
    Dim ws構成マスタ As Worksheet
    Set ws逆展開 = Worksheets(STRシート名_逆展開)
    Set ws構成マスタ = Worksheets(STRシート名_構成マスタ)

    '逆展開シートの子コードを変数を取得
    With ws逆展開
        Dim str子_検索対象_品目コード As String
        str子_検索対象_品目コード = .Cells(逆展開_検索対象_入力_ROW, 逆展開_検索対象_コード_COL).Value
    End With

    Dim rng As Range
    Set rng = ws構成マスタ.Range("M:M").Find(str子_検索対象_品目コード)

    If Not (rng Is Nothing) Then
      '構成マスタに登録が無ければ次の処理へ
    Else
        MsgBox "[" & str子_検索対象_品目コード & " ]は、構成マスタ内で子としての登録がありません"
        End
    End If
Application.ScreenUpdating = True
End Sub


Sub 逆展開_前回の検索結果が残っていれば削除()
Application.ScreenUpdating = False
    Dim ws逆展開 As Worksheet
    Dim ws構成マスタ As Worksheet
    Set ws逆展開 = Worksheets(STRシート名_逆展開)
    Set ws構成マスタ = Worksheets(STRシート名_構成マスタ)

    '使用品目（親）の親_品目コードを変数
    With ws逆展開
        Dim str子_前回検索結果_親品目コード As String
               str子_前回検索結果_親品目コード = .Cells(逆展開_展開先_先頭_ROW, 逆展開_検索対象_コード_COL).Value

      If str子_前回検索結果_親品目コード = "" Then
        '前回の検索結果データが残っていなければ次の処理へ
      Else
        '品目コードの最終行を上方向に取得
        Dim lng品目コード_最終行 As Long
        lng品目コード_最終行 = .Cells(Rows.Count, 逆展開_検索対象_コード_COL).End(xlUp).Row
        '先頭から最終までの行を削除
        .Range(逆展開_展開先_先頭_ROW & ":" & lng品目コード_最終行).Delete
      End If
    End With
  
Application.ScreenUpdating = True
End Sub

Sub 逆展開_1階層目の呼び出し()
Application.ScreenUpdating = False

    Dim ws逆展開 As Worksheet
    Dim ws構成マスタ As Worksheet
    Set ws逆展開 = Worksheets(STRシート名_逆展開)
    Set ws構成マスタ = Worksheets(STRシート名_構成マスタ)

    '検索するデータを配列　myData　に格納する。
    Dim 構成マスタ_last_row As Long
    Dim var配列_構成マスタ() As Variant
    Dim i As Long
    Dim cn As Long

    '検索するデータを配列に格納する。
    With Sheets(STRシート名_構成マスタ)
          構成マスタ_last_row = .Cells(Rows.Count, 1).End(xlUp).Row
          var配列_構成マスタ = .Range(.Cells(2, 1), .Cells(構成マスタ_last_row, 構成マスタ_LAST_COL)).Value
    End With

    '検索する子コードを変数にセット
    With Sheets(STRシート名_逆展開)
        Dim str検索_子_コード As String
        str検索_子_コード = Cells(逆展開_検索対象_入力_ROW, 逆展開_検索対象_コード_COL).Value
    End With

    '配列　var配列_構成マスタ　の中で検索で一致したデータを配列　var検索結果_配列　に格納する。
    '検索条件　：検索対象コードが、親子区分2(子)の主キーに登録されている
    '配列再格納：子の情報「構成連番、構成数、構成項目1・2」　親の情報「品目コード」
    ReDim var検索結果_配列(1 To 構成マスタ_last_row, 1 To 構成マスタ_LAST_COL)
    For i = LBound(var配列_構成マスタ) To UBound(var配列_構成マスタ) - 1
        If var配列_構成マスタ(i, 構成マスタ_主キー_構成コード_COL) Like "*" & str検索_子_コード & "*" _
              And var配列_構成マスタ(i, 構成マスタ_親子区分_COL) Like "*" & "2" & "*" Then
          cn = cn + 1
          
          var検索結果_配列(cn, 1) = var配列_構成マスタ(i, 構成マスタ_構成連番_COL)
          var検索結果_配列(cn, 2) = var配列_構成マスタ(i, 構成マスタ_親_品目コード_COL)
          var検索結果_配列(cn, 3) = var配列_構成マスタ(i, 構成マスタ_構成数_COL)
          var検索結果_配列(cn, 4) = var配列_構成マスタ(i, 構成マスタ_構成歩留り率_COL)
          var検索結果_配列(cn, 5) = var配列_構成マスタ(i, 構成マスタ_構成項目1_COL)
          var検索結果_配列(cn, 6) = var配列_構成マスタ(i, 構成マスタ_構成項目2_COL)

        End If
    Next i

    '検索結果をセルに出力
    With ws逆展開
        '列ごとに配列格納データを出力
        Dim j As Long
        For j = 1 To cn
          .Cells(逆展開_展開先_項目_ROW + j, 逆展開_展開先_構成連番_COL).Value = var検索結果_配列(j, 1)
          .Cells(逆展開_展開先_項目_ROW + j, 逆展開_品目コード_COL).Value = var検索結果_配列(j, 2)
          .Cells(逆展開_展開先_項目_ROW + j, 逆展開_展開先_子_構成数_COL).Value = var検索結果_配列(j, 3)
          .Cells(逆展開_展開先_項目_ROW + j, 逆展開_展開先_子_歩留り率_COL).Value = var検索結果_配列(j, 4)
          .Cells(逆展開_展開先_項目_ROW + j, 逆展開_展開先_子_項目1_COL).Value = var検索結果_配列(j, 5)
          .Cells(逆展開_展開先_項目_ROW + j, 逆展開_展開先_子_項目2_COL).Value = var検索結果_配列(j, 6)
        Next j
          
        cn = cn - 1 '一個余分なので消す

        '共通する項目を入力
        .Range(Cells(逆展開_展開先_先頭_ROW, 逆展開_展開先_展開チェック_COL), Cells(逆展開_展開先_先頭_ROW + cn, 逆展開_展開先_展開チェック_COL)).Value = 1
        .Range(Cells(逆展開_展開先_先頭_ROW, 逆展開_展開先_子_品目コード_COL), Cells(逆展開_展開先_先頭_ROW + cn, 逆展開_展開先_子_品目コード_COL)).Value = _
            str検索_子_コード
        .Range(Cells(逆展開_展開先_先頭_ROW, 逆展開_展開先_階層_COL), Cells(逆展開_展開先_先頭_ROW + cn, 逆展開_展開先_階層_COL)).Value = 1

    End With
Application.ScreenUpdating = True
End Sub



Sub 逆展開_検索対象の2階層以降の情報を抽出()
Application.ScreenUpdating = False

    Dim ws逆展開 As Worksheet
    Dim ws構成マスタ As Worksheet
    Set ws逆展開 = Worksheets(STRシート名_逆展開)
    Set ws構成マスタ = Worksheets(STRシート名_構成マスタ)

    '検索するデータを配列　myData　に格納する。
    Dim 構成マスタ_last_row As Long
    Dim var配列_構成マスタ() As Variant
    Dim i As Long
    Dim cn As Long

    '構成マスタデータを配列に格納する。検索しやすくするため。
     With ws構成マスタ
          構成マスタ_last_row = .Cells(Rows.Count, 1).End(xlUp).Row
          var配列_構成マスタ = .Range(.Cells(2, 1), .Cells(構成マスタ_last_row, 構成マスタ_LAST_COL)).Value
    End With

    '検索対象のコードを変数にセット、展開先の先頭から検索を始める
    With ws逆展開
        Dim str検索_品目コード As String
        Dim j As Long
        j = 逆展開_展開先_先頭_ROW

        Do '★品目コードが空欄になったらループを抜ける
        str検索_品目コード = .Cells(j, 逆展開_品目コード_COL).Value
        
        '検索対象の品目コードが構成マスタの子コード列に無い場合、末端フラグを立てて次の処理へ
        'あれば階層を深めて検索
        Dim rng As Range
        Set rng = ws構成マスタ.Range("M:M").Find(str検索_品目コード)
        If (rng Is Nothing) Then
            .Cells(j, 逆展開_展開先_末端コード_COL).Value = 1
        Else
            '配列　var配列_構成マスタ　の中で検索で一致したデータを配列　var検索結果_配列　に格納する。
            cn = 0 '一回ごとにリセット
            ReDim var検索結果_配列(1 To 構成マスタ_last_row, 1 To 構成マスタ_LAST_COL)
            For i = LBound(var配列_構成マスタ) To UBound(var配列_構成マスタ)
                If var配列_構成マスタ(i, 構成マスタ_主キー_構成コード_COL) Like "*" & str検索_品目コード & "*" _
                    And var配列_構成マスタ(i, 構成マスタ_親子区分_COL) Like "*" & "2" & "*" Then
                    
                  cn = cn + 1
                  var検索結果_配列(cn, 1) = var配列_構成マスタ(i, 構成マスタ_構成連番_COL)
                  var検索結果_配列(cn, 2) = var配列_構成マスタ(i, 構成マスタ_親_品目コード_COL)
                  var検索結果_配列(cn, 3) = var配列_構成マスタ(i, 構成マスタ_構成数_COL)
                  var検索結果_配列(cn, 4) = var配列_構成マスタ(i, 構成マスタ_構成歩留り率_COL)
                  var検索結果_配列(cn, 5) = var配列_構成マスタ(i, 構成マスタ_構成項目1_COL)
                  var検索結果_配列(cn, 6) = var配列_構成マスタ(i, 構成マスタ_構成項目2_COL)
            
                End If
            Next i
               
            If var検索結果_配列(1, 1) = "" Then '配列が空なら（末端コードなら）1をつける
                   .Cells(j, 逆展開_展開先_末端コード_COL).Value = 1
            Else
              '最後の空白行を消す。
              cn = cn
              
              '複数行したに挿入する
              Dim row_first As Long
              Dim row_last As Long
              row_first = j + 1
              row_last = j + cn
              .Range(Rows(row_first), Rows(row_last)).Insert CopyOrigin:=xlFormatFromLeftOrAbove

          '検索結果をセルに出力
                  '列ごとに配列格納データを出力
              Dim k As Long
                For k = 1 To cn
                  .Cells(j + k, 逆展開_展開先_構成連番_COL).Value = var検索結果_配列(k, 1)
                  .Cells(j + k, 逆展開_品目コード_COL).Value = var検索結果_配列(k, 2)
                  .Cells(j + k, 逆展開_展開先_子_構成数_COL).Value = var検索結果_配列(k, 3)
                  .Cells(j + k, 逆展開_展開先_子_歩留り率_COL).Value = var検索結果_配列(k, 4)
                  .Cells(j + k, 逆展開_展開先_子_項目1_COL).Value = var検索結果_配列(k, 5)
                  .Cells(j + k, 逆展開_展開先_子_項目2_COL).Value = var検索結果_配列(k, 6)
                Next k

              '展開チェックから階層までを個々に入力
              Range(Cells(row_first, 逆展開_展開先_展開チェック_COL), Cells(row_last, 逆展開_展開先_展開チェック_COL)) = 1
              Range(Cells(row_first, 逆展開_展開先_子_品目コード_COL), Cells(row_last, 逆展開_展開先_子_品目コード_COL)) = str検索_品目コード
              Range(Cells(row_first, 逆展開_展開先_階層_COL), Cells(row_last, 逆展開_展開先_階層_COL)) = Cells(j, 逆展開_展開先_階層_COL).Value + 1
            End If
          End If
          j = j + 1
        Loop Until .Cells(j, 逆展開_品目コード_COL).Value = "" '★品目コードが空欄になったらループを抜ける
    End With

Application.ScreenUpdating = True
End Sub
Sub 逆展開_品目コード一括検索()
Application.ScreenUpdating = False
    
    Dim ws逆展開 As Worksheet
    Dim ws品目マスタ As Worksheet
        Set ws逆展開 = Worksheets(STRシート名_逆展開)
        Set ws品目マスタ = Worksheets(STRシート名_品目マスタ)
    
    
    '検索するデータを配列に格納する。
    With ws品目マスタ
        Dim var配列_品目マスタ As Variant
        Dim 品目マスタ_last_row As Long
            品目マスタ_last_row = .Cells(Rows.Count, 1).End(xlUp).Row
             var配列_品目マスタ = .Range(.Cells(2, 1), .Cells(品目マスタ_last_row, 品目マスタ_LAST_COL)).Value
    End With
    
    Dim str品目コード As String
    Dim 該当コード_row As Long
    Dim i As Long
    Dim j As Long

With ws逆展開
        Dim 逆展開_last_row As Long
            逆展開_last_row = .Cells(Rows.Count, 1).End(xlUp).Row
   
    '逆展開の親コードを1件づつ確認していきます。
    For j = 逆展開_展開先_先頭_ROW To 逆展開_last_row
        str品目コード = .Cells(j, 逆展開_品目コード_COL).Value
         If str品目コード <> "" Then    '検索対象コードが空欄でないなら、配列から探す
         
            For i = LBound(var配列_品目マスタ) To UBound(var配列_品目マスタ)      'ループ処理で配列を検索し、行数を変数へ格納
            If var配列_品目マスタ(i, 品目マスタ_コード_COL) = str品目コード Then   '条件に一致するコードが見つかったら
                '構成登録に、品目マスタの値をセットし、ループ処理iから抜けてjのループへ

                     .Cells(j, 逆展開_品目名_COL).Value = var配列_品目マスタ(i, 品目マスタ_品名_COL)
                     .Cells(j, 逆展開_大分類_COL).Value = var配列_品目マスタ(i, 品目マスタ_大分類_COL)
                     .Cells(j, 逆展開_中分類_COL).Value = var配列_品目マスタ(i, 品目マスタ_中分類_COL)
                     .Cells(j, 逆展開_小分類_COL).Value = var配列_品目マスタ(i, 品目マスタ_小分類_COL)
                     .Cells(j, 逆展開_使用停止フラグ_COL).Value = var配列_品目マスタ(i, 品目マスタ_使用停止フラグ_COL)
                     
                 Exit For    '該当コードが見つかったら、書き込んで検索終了(ループから抜ける)
            End If
        Next i
    End If
    Next j
    
    '逆展開の子コードを1件づつ確認していきます。
    For j = 逆展開_展開先_先頭_ROW To 逆展開_last_row
        str品目コード = .Cells(j, 逆展開_展開先_子_品目コード_COL).Value
         If str品目コード <> "" Then    '検索対象コードが空欄でないなら、配列から探す
         
            For i = LBound(var配列_品目マスタ) To UBound(var配列_品目マスタ)      'ループ処理で配列を検索し、行数を変数へ格納
            If var配列_品目マスタ(i, 品目マスタ_コード_COL) = str品目コード Then   '条件に一致するコードが見つかったら
                '構成登録に、品目マスタの値をセットし、ループ処理iから抜けてjのループへ

                     .Cells(j, 逆展開_展開先_子_品目名_COL).Value = var配列_品目マスタ(i, 品目マスタ_品名_COL)
                     .Cells(j, 逆展開_展開先_子_構成単位_COL).Value = var配列_品目マスタ(i, 品目マスタ_単位_COL)
                     
                 Exit For    '該当コードが見つかったら、書き込んで検索終了(ループから抜ける)
            End If
        Next i
    End If
    Next j
End With

Application.ScreenUpdating = True
End Sub

