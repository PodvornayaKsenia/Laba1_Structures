#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <queue>

#include "stack.h"
#include "single_list.h"
#include "double_list.h"
#include "queue.h"
#include "avl_tree.h"
#include "array.h"

using namespace std;

SNode* g_stack = nullptr;
Node* g_slist = nullptr;
DNode* g_dlist_head = nullptr;
DNode* g_dlist_tail = nullptr;
MyQueue g_queue;
AVLNode* g_tree = nullptr;
MyArray g_array;

struct Query {
    string command;
    string name;
    string target;
    string value;
    string extra;
    int number;
};

bool parseArguments(int argc, char* argv[], string& filename, string& text) {
    if (argc != 5) return false;
    if (string(argv[1]) != "--file" || string(argv[3]) != "--query") return false;
    filename = argv[2];
    text = argv[4];
    return true;
}

bool parseQuery(const string& text, Query& query) {
    istringstream ss(text);
    if (!(ss >> query.command >> query.name)) return false;

    string arg1, arg2, arg3;
    if (ss >> arg1) {
        try { query.number = stoi(arg1); } catch (...) { query.number = 0; }
        query.target = arg1; 
        
        if (ss >> arg2) {
            query.value = arg2;
            if (ss >> arg3) {
                query.extra = arg3;
            }
        } else {
            query.value = arg1;
        }
    } else {
        query.target = "";
        query.value = "";
        query.extra = "";
        query.number = 0;
    }
    return true;
}

bool readFile(const string& filename, const Query& query, vector<string>& lines, vector<string>& values, int& position, char& type) {
    position = -1;
    type = query.command.empty() ? '?' : query.command[0];
    ifstream input(filename);
    if (!input.is_open()) return true;

    string line;
    while (getline(input, line)) {
        istringstream row(line);
        string savedType, savedName;
        if (row >> savedType >> savedName && savedName == query.name) {
            type = savedType[0];
            position = lines.size();
            string val;
            while (row >> val) values.push_back(val);
        }
        lines.push_back(line);
    }
    return true;
}

void loadStructure(char type, const vector<string>& values) {
    if (type == 'S') { for (int i = values.size() - 1; i >= 0; --i) SPush(g_stack, values[i]); }
    else if (type == 'F') { for (const string& v : values) pushBack(g_slist, v); }
    else if (type == 'L') { for (const string& v : values) LPushBack(g_dlist_head, g_dlist_tail, v); }
    else if (type == 'Q') { for (const string& v : values) QPush(g_queue, v); }
    else if (type == 'T') { for (const string& v : values) TInsert(g_tree, stoi(v)); }
    else if (type == 'M') { for (const string& v : values) MPushBack(g_array, v); }
}

string structureToString(char type) {
    ostringstream out;
    if (type == 'S') { SNode* cur = g_stack; while (cur) { out << cur->data << " "; cur = cur->next; } }
    else if (type == 'F') { Node* cur = g_slist; while (cur) { out << cur->data << " "; cur = cur->next; } }
    else if (type == 'L') { DNode* cur = g_dlist_head; while (cur) { out << cur->data << " "; cur = cur->next; } }
    else if (type == 'Q') { QNode* cur = g_queue.head; while (cur) { out << cur->data << " "; cur = cur->next; } }
    else if (type == 'T') { 
        queue<AVLNode*> q;
        if (g_tree) q.push(g_tree);
        while (!q.empty()) {
            AVLNode* n = q.front(); q.pop();
            out << n->data << " ";
            if (n->left) q.push(n->left);
            if (n->right) q.push(n->right);
        }
    }
    else if (type == 'M') { 
        for (int i = 0; i < g_array.size; i++) out << g_array.data[i] << " "; 
    }
    return out.str();
}

bool writeFile(const string& filename, vector<string>& lines, int position, char type, const string& name) {
    string updated = string(1, type) + " " + name + " " + structureToString(type);
    if (position == -1) lines.push_back(updated);
    else lines[position] = updated;

    ofstream output(filename);
    if (!output) return false;
    for (const auto& line : lines) output << line << '\n';
    return true;
}

bool executeCommand(char type, const Query& query, bool& changed) {
    changed = false;
    const string& cmd = query.command;

    if (cmd == "PRINT") {
        if (type == 'S') SPrint(g_stack);
        else if (type == 'F') printList(g_slist);
        else if (type == 'L') LPrintForward(g_dlist_head);
        else if (type == 'Q') QPrint(g_queue);
        else if (type == 'T') TPrintInOrder(g_tree);
        else if (type == 'M') MPrint(g_array);
        else cout << "ERROR: Structure not found\n";
        return true;
    }

    if (cmd == "SPUSH") { SPush(g_stack, query.value); cout << query.value << "\n"; changed = true; }
    else if (cmd == "SPOP") { 
        string val = SPop(g_stack); 
        if (!val.empty()) {
            cout << val << "\n";    
        } else {
            cout << "Stack is empty\n";
        }
        changed = true; 
    }

    else if (cmd == "FPUSH") { 
        if (query.target == "front") pushFront(g_slist, query.value);
        else if (query.target == "back") pushBack(g_slist, query.value);
        else if (query.target == "after") insertAfter(g_slist, query.value, query.extra);
        else if (query.target == "before") insertBefore(g_slist, query.value, query.extra);
        cout << (query.extra.empty() ? query.value : query.extra) << "\n"; changed = true; 
    }
    else if (cmd == "FDEL") { 
        if (query.target == "front") popFront(g_slist);
        else if (query.target == "back") popBack(g_slist);
        else if (query.target == "after") deleteAfter(g_slist, query.value);
        else if (query.target == "before") deleteBefore(g_slist, query.value);
        else removeByValue(g_slist, query.value);
        cout << "OK\n"; changed = true; 
    }
    else if (cmd == "FGET") { printList(g_slist); }
    else if (cmd == "FIND") { 
        if (search(g_slist, query.value) != nullptr) cout << "TRUE\n";
        else cout << "FALSE\n";
    }

    else if (cmd == "LPUSH") { 
        if (query.target == "front") LPushFront(g_dlist_head, g_dlist_tail, query.value);
        else if (query.target == "back") LPushBack(g_dlist_head, g_dlist_tail, query.value);
        else if (query.target == "after") LInsertAfter(g_dlist_head, g_dlist_tail, query.value, query.extra);
        else if (query.target == "before") LInsertBefore(g_dlist_head, g_dlist_tail, query.value, query.extra);
        cout << (query.extra.empty() ? query.value : query.extra) << "\n"; changed = true; 
    }
    else if (cmd == "LDEL") { 
        if (query.target == "front") LPopFront(g_dlist_head, g_dlist_tail);
        else if (query.target == "back") LPopBack(g_dlist_head, g_dlist_tail);
        else if (query.target == "after") LDeleteAfter(g_dlist_head, g_dlist_tail, query.value);
        else if (query.target == "before") LDeleteBefore(g_dlist_head, g_dlist_tail, query.value);
        else LRemoveByValue(g_dlist_head, g_dlist_tail, query.value);
        cout << "OK\n"; changed = true; 
    }
    else if (cmd == "LGET") { LPrintForward(g_dlist_head); }
    
    else if (cmd == "LSEARCH") { 
        if (LSearch(g_dlist_head, query.value)) cout << "TRUE\n";
        else cout << "FALSE\n";
    }

    else if (cmd == "QPUSH") { QPush(g_queue, query.value); cout << query.value << "\n"; changed = true; }
    else if (cmd == "QPOP") { 
        string val = QPop(g_queue); 
        if (!val.empty()) {
            cout << val << "\n";    
        } else {
            cout << "Queue is empty\n";
        }
        changed = true; 
    }
    else if (cmd == "QGET") { QPrint(g_queue); }

    else if (cmd == "TINSERT") { TInsert(g_tree, query.number); cout << query.number << "\n"; changed = true; }
    else if (cmd == "TDEL") { TRemove(g_tree, query.number); cout << "OK\n"; changed = true; }
    else if (cmd == "TGET") { TPrintInOrder(g_tree); cout << "\n"; }
    else if (cmd == "TPRE") { TPrintPreOrder(g_tree); cout << "\n"; }
    else if (cmd == "TPOST") { TPrintPostOrder(g_tree); cout << "\n"; }
    else if (cmd == "TLEVEL") { TPrintLevelOrder(g_tree); cout << "\n"; }
    else if (cmd == "ISMEMBER") { 
        if (TSearch(g_tree, query.number)) cout << "TRUE\n";
        else cout << "FALSE\n";
    }

    else if (cmd == "MPUSH") { MPushBack(g_array, query.value); cout << query.value << "\n"; changed = true; }
    else if (cmd == "MINSERT") { MInsert(g_array, query.number, query.value); cout << query.value << "\n"; changed = true; }
    else if (cmd == "MGET") { cout << MGet(g_array, query.number) << "\n"; }
    else if (cmd == "MDEL") { MRemove(g_array, query.number); cout << "OK\n"; changed = true; }
    else if (cmd == "MSET") { MReplace(g_array, query.number, query.value); cout << query.value << "\n"; changed = true; }
    else if (cmd == "MLEN") { cout << MLength(g_array) << "\n"; }

    else { cout << "ERROR: Unknown command\n"; return false; }
    return true;
}

int main(int argc, char* argv[]) {
    string filename, text;
    Query query;

    if (!parseArguments(argc, argv, filename, text)) {
        cerr << "Формат: ./dbms --file file.data --query 'COMMAND name [args]'\n";
        return 1;
    }
    if (!parseQuery(text, query)) {
        cerr << "Ошибка разбора команды\n";
        return 1;
    }

    MInit(g_array);

    vector<string> lines;
    vector<string> values;
    int position;
    char type;

    readFile(filename, query, lines, values, position, type);
    loadStructure(type, values);

    bool changed;
    bool success = executeCommand(type, query, changed);

    if (success && changed) writeFile(filename, lines, position, type, query.name);

    return success ? 0 : 1;
}
