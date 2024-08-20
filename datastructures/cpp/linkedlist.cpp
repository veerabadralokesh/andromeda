
#include <iostream>

using namespace std;

struct Node {
    int val;
    // in c
    // struct Node* next;

    // in c++
    Node* next;
};

struct ListNode {
    int val;
    ListNode *next;
    ListNode(): val(0), next(nullptr) {}
    ListNode(int x): val(x), next(nullptr) {}
    ListNode(int x, ListNode *next): val(x), next(next) {}
};

int main() {

    cout << "Linked list" << endl;

    // c
    // Node* temp = (Node*)malloc(sizeof(Node));

    ListNode head(0);
    ListNode *dummy = &head;
    for (int i=1; i<10; i++) {
        dummy->next = new ListNode(i);
        dummy = dummy->next;
    }
    dummy = &head;
    while (true) {
        cout << dummy->val;
        dummy = dummy->next;
        if (dummy != nullptr){
            cout << " -> ";
        } else {
            break;
        }
    }
    cout << endl;

    return 0;
}