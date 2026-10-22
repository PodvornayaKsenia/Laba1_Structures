#ifndef AVL_TREE_H
#define AVL_TREE_H

#include <iostream>
#include <algorithm>
#include <queue>

using namespace std;

struct AVLNode {
    int data;
    int height;
    AVLNode *left, *right;
    AVLNode(int val);
};

void TInsert(AVLNode* &n, int val);
void TRemove(AVLNode* &n, int val);
bool TSearch(AVLNode* n, int val);

void TPrintInOrder(AVLNode* n);    
void TPrintPreOrder(AVLNode* n);   
void TPrintPostOrder(AVLNode* n); 
void TPrintLevelOrder(AVLNode* n); 

#endif
