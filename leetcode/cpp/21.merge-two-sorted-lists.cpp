/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode dummy = ListNode(0);
        ListNode *head = &dummy;
        int a, b;
        while (list1 != nullptr || list2 != nullptr) {
            a = INT_MAX, b = INT_MAX;
            if (list1 != nullptr) {
                a = list1->val;
            }
            if (list2 != nullptr) {
                b = list2->val;
            }
            if (a < b) {
                head->next = new ListNode(a);
                list1 = list1->next;
            } else {
                head->next = new ListNode(b);
                list2 = list2->next;
            }
            head = head->next;
        }
        return dummy.next;
    }
};

/* */

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode* tail = new ListNode(0);
        ListNode* head = tail;
        int l1,l2;
        while(list1 && list2){
            if(list1->val < list2->val){
                tail->next = list1;
                list1 = list1->next;
            }
            else{
                tail->next = list2;
                list2 = list2->next;
            }
            tail = tail->next;
        }
        if(list2){
            tail->next = list2;
        }
        else if(list1){
            tail->next = list1;
        }
        return head->next;
    }
};

