int main() {
  volatile int j = 0;
  for(int i = 0; i < 100; i++){
    j += j + 1;
  }
}
