This Is First Rust Project 

This Project is created for practice and it is available on official rust Book on the website 
In this Project we are generating a random number between 1 to 100
For that we have used external library named rand  from the official comunity of rust 
so after generating random number we are string it in integer 
and taking user input to guess the random number we are taking the input as string and after getting
the input we have used trim function to remove empty spaces and new lines . and after that we are storing 
that string after converting in integer into integer element for that we have to remove all the 
other sing present in strings like special char and many more , for that we have used parse() function
which will help us to check wheather string is containing only number or not if yes then it will give true ,
and store in other variable which is interger and if no then it will return false and take the input again .
after that we are comparing both random number and unser input 
if user input is greater that random number then it will return Input is High ,and take input again,
if it is lower than random number then it will return Input Is low , and take inout again 
and if it is equal then return congratulations you won .
