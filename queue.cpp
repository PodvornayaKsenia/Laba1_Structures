#include "queue.h"
//вставка

void QPush(MyQueue& q, string val) {
    QNode* newNode = new QNode(val);
    if (q.tail == nullptr) {
        q.head = q.tail = newNode;
        return;
    }
    q.tail->next = newNode;
    q.tail = newNode;
}
//удаление
string QPop(MyQueue& q) {
    if (q.head == nullptr) return "";
    
    QNode* toDelete = q.head;
    string val = toDelete->data;
    
    q.head = q.head->next;
    if (q.head == nullptr) {
        q.tail = nullptr;
    }
    
    delete toDelete;
    return val;
}
//вывод
void QPrint(const MyQueue& q) {
    if (q.head == nullptr) {
        cout << "[Очередь пуста]" << endl;
        return;
    }
    QNode* temp = q.head;
    while (temp) {
        cout << temp->data << endl; 
        temp = temp->next;
    }
}
