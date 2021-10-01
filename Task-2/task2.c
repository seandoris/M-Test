typedef struct node {
  int data;
  struct node * right, * left;
} NODE;

void insert (NODE *Prev, NODE *New){
    (*Prev).right = New;
    (*New).left = Prev;
};

int main(){

    return 0;
}