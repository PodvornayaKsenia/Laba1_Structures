#include "avl_tree.h"

AVLNode::AVLNode(int val) : data(val), height(1), left(nullptr), right(nullptr) {}
//вспомогательные
int getHeight(AVLNode* n) { return n ? n->height : 0; }
int getBalance(AVLNode* n) { return n ? getHeight(n->left) - getHeight(n->right) : 0; }
void updateHeight(AVLNode* n) { if (n) n->height = 1 + max(getHeight(n->left), getHeight(n->right)); }
//повороты
void rotateRight(AVLNode* &y) {
    AVLNode* x = y->left;
    AVLNode* T2 = x->right;
    x->right = y;
    y->left = T2;
    updateHeight(y);
    updateHeight(x);
    y = x;
}

void rotateLeft(AVLNode* &x) {
    AVLNode* y = x->right;
    AVLNode* T2 = y->left;
    y->left = x;
    x->right = T2;
    updateHeight(x);
    updateHeight(y);
    x = y;
}

void balance(AVLNode* &node) {
    if (!node) return;
    updateHeight(node);
    int bf = getBalance(node);
    if (bf > 1 && getBalance(node->left) >= 0) rotateRight(node);
    else if (bf < -1 && getBalance(node->right) <= 0) rotateLeft(node);
    else if (bf > 1 && getBalance(node->left) < 0) { rotateLeft(node->left); rotateRight(node); }
    else if (bf < -1 && getBalance(node->right) > 0) { rotateRight(node->right); rotateLeft(node); }
}

//вставка
void TInsert(AVLNode* &n, int val) {
    if (!n) { n = new AVLNode(val); return; }
    if (val < n->data) TInsert(n->left, val);
    else if (val > n->data) TInsert(n->right, val);
    else return; 
    balance(n);
}

//поиск эл-та
bool TSearch(AVLNode* n, int val) {
    if (!n) return false;
    if (n->data == val) return true;
    return val < n->data ? TSearch(n->left, val) : TSearch(n->right, val);
}

//поиск мин. узла в поддереве
AVLNode* findMin(AVLNode* n) {
    while (n && n->left) n = n->left;
    return n;
}

//удаление
void TRemove(AVLNode* &n, int val) {
    if (!n) return;
    if (val < n->data) TRemove(n->left, val);
    else if (val > n->data) TRemove(n->right, val);
    else {
        if (!n->left || !n->right) {
            AVLNode* temp = n->left ? n->left : n->right;
            delete n;
            n = temp;
        } else {
            AVLNode* temp = findMin(n->right);
            n->data = temp->data;
            TRemove(n->right, temp->data);
        }
    }
    if (n) balance(n);
}


//read
void TPrintInOrder(AVLNode* n) {
    if (n) { 
        TPrintInOrder(n->left); 
        cout << n->data << " "; 
        TPrintInOrder(n->right); 
    }
}

void TPrintPreOrder(AVLNode* n) {
    if (n) {
        cout << n->data << " ";
        TPrintPreOrder(n->left);
        TPrintPreOrder(n->right);
    }
}

void TPrintPostOrder(AVLNode* n) {
    if (n) {
        TPrintPostOrder(n->left);
        TPrintPostOrder(n->right);
        cout << n->data << " ";
    }
}

void TPrintLevelOrder(AVLNode* root) {
    if (!root) return;
    queue<AVLNode*> q;
    q.push(root);
    while (!q.empty()) {
        AVLNode* temp = q.front(); q.pop();
        cout << temp->data << " ";
        if (temp->left) q.push(temp->left);
        if (temp->right) q.push(temp->right);
    }
}
