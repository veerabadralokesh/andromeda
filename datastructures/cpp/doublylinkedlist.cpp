
#include <iostream>

using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode *prev;
    ListNode(): val(0), next(nullptr), prev(nullptr) {}
    ListNode(int x): val(x), next(nullptr), prev(nullptr) {}
    ListNode(int x, ListNode *prev): val(x), prev(prev), next(nullptr) {}
    ListNode(int x, ListNode *next, ListNode * prev): val(x), next(next), prev(prev) {}
};

int main() {

    cout << "Doubly Linked list" << endl;

    ListNode head(0);
    ListNode *dummy = &head;
    for (int i=1; i<10; i++) {
        dummy->next = new ListNode(i, dummy);
        dummy = dummy->next;
    }
    dummy = &head;
    while (true) {
        cout << dummy->val;
        if (dummy->next != nullptr){
            dummy = dummy->next;
            cout << " -> ";
        } else {
            break;
        }
    }
    cout << endl;
    while (true) {
        cout << dummy -> val;
        if (dummy->prev != nullptr) {
            dummy = dummy->prev;
            cout << " <- ";
        } else {
            break;
        }
    }
    cout << endl;

    return 0;
}
