cd ..;
cargo build;
cd -;
cp ../target/debug/tinyssh ./;
clang deamon.c -o deamon;
clang patchr.c -o patchr;
./patchr tinyssh deamon;