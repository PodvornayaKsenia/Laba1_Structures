#ifndef QUEUE_H
#define QUEUE_H

#include <string>
#include <iostream>

using namespace std;

struct QNode {
    string data;
    QNode* next;
    QNode(string val) : data(val), next(nullptr) {}
};

struct MyQueue {
    QNode* head = nullptr;
    QNode* tail = nullptr;
};

void QPush(MyQueue& q, string val);
string QPop(MyQueue& q);
void QPrint(const MyQueue& q);

#endif
