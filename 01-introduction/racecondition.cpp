#include <vector>
#include <iostream>

int main(){
  std::vector<int> v = {10,20,30,40,50};
  for(int i: v){
    for(int x = 0; x < i; x++){
      v.push_back(x);
    }
    std::cout << i << ' ' ;
  }
}
