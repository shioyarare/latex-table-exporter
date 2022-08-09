# latex-table-exporter
空白区切りのファイルを `LaTeX` の表形式に変換するプログラムの

## 起動方法
```
$ latex-table-exporter <ファイル名>
```
### inputファイル形式
```
a b c d
1 2 3 4
5 6 7 8
```
### 標準出力output
```
\begin{table}{H}
  \caption{title}
  \label{tab:data}
  \centering
  \begin{tabular}{cccc}
    \hline
    a & b & c & d \\
    \hline \hline
    1 & 2 & 3 & 4 \\
    5 & 6 & 7 & 8 \\
    \hline
  \end{tabular}
\end{table}
```
## その他
### `#` 文字はコメントとして認識されます
```
a b c d
# コメントできるよ！
1 2 3 4
5 6 7 8
```
### 空白からなる行は読み飛ばされます(見やすいように整形可能)
```
a b c d

1 2 3 4
5 6 7 8
```
