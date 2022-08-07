# latex-table-exporter
空白区切りのファイルを `LaTeX` の表形式に変換
- input
```
a b c d
1 2 3 4
5 6 7 8
```
- output
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
