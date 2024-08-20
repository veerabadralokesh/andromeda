
#include <iostream>
#include <vector>

using namespace std;

const int MAX_SIZE = 1001;

struct Stack {
    private:
        int top = -1;
        vector<int> arr;
        int maxLen = MAX_SIZE;
    public:
        Stack(): arr(MAX_SIZE) {}
        Stack(int len): arr(len), maxLen(0) {}
        void Push(int x) {
            if (top == maxLen-1) {
                cout << "Error: stack overflow" << endl;
                return;
            }
            arr[++top] = x;
        }
        int Top() {
            if (top == -1) {
                cout << "Error: stack empty" << endl;
                return -1;
            }
            return arr[top];
        }
        int Pop() {
            int ret = Top();
            top--;
            return ret;
        }
        int Len() {
            return top + 1;
        }
};

int main() {

    Stack stack;

    for (int i=0; i < 10; i++) {
        stack.Push(i);
    }
    cout << "Stack length: " << stack.Len() << endl;

    while (stack.Len()) {
        cout << stack.Pop() << endl;
    }

    cout << stack.Pop() << endl;

    return 0;
}
