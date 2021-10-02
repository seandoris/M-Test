# Motorola-test
ZC Exercises
C/C++ libvirt exercise

Write a small binary in C/C++ using the C libraries from libvirt to 

    Connect to the QEmu (KVM) running on the local machine

    List all virtual machines and their states. 


The goal is to recreate the functionality of the usual bash command “virsh list --all”




C/C++ Double linked list exercise

The following structure represents a node in a double linked list

typedef struct node {
  int data;
  struct node * right, * left;
} NODE;

A function declared as shown below is to insert a new node in such a list to the right of the node pointed by ‘Prev’.

void insert (NODE *Prev, NODE *New);

How would you implement the ‘insert’ function? 

Can you write the code to implement the insert function?






RUST Library exercise

Write a small Rust application that simulates a library:

    The application shall read a configuration file containing information about what books are available at the library and if they are lent out.

    The user should be able to interact with the program via the command line.

    The user can borrow a book if that is not currently lent out.

    The user can return a book if it is lent out.

    Upon exiting the application, the configuration file should be updated according to the state of the books at the end of the session.


_END_
