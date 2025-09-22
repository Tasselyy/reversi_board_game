# simple test case:

Black Move 1: Input "ef" (standard f5) 
White Move 1: Input "df" (standard f4)
Black Move 2: Input "cd" (standard d3)
White Move 2: Input "fd" (standard d6)
Black Move 3: Input "gd" (standard d7)
White Move 3: Input "ce" (standard e3)
Black Move 4: Input "cf" (standard f3)
White Move 4: Input "ec" (standard c5)
Black Move 5: Input "eb" (standard b5)
Expected Pieces: ~13 total (4 initial + 9 placed);

# test command example:

``` cmd
Get-Content .\test\inputs\in140.txt | cargo run | Out-File .\test\my_outputs\my_out140.txt

Compare-Object (Get-Content .\test\my_outputs\my_out140.txt) (Get-Content .\test\outputs\out140.txt)
```

