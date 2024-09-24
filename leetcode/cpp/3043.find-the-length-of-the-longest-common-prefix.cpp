
#define NUMBERS 10

class TrieNode {
   public:
    TrieNode *childNodes[NUMBERS];
    bool isWordEnd;
    TrieNode() {
        isWordEnd = false;
        for (int i = 0; i < NUMBERS; i++) {
            childNodes[i] = nullptr;
        }
    }
};

class Trie {
   private:
    TrieNode *root;

   public:
    Trie() {
        root = new TrieNode();
    }

    void insert(const string &key) {
        TrieNode *p = root;
        for (int i = 0; i < key.length(); i++) {
            int j = key[i] - '0';
            if (!p->childNodes[j]) {
                p->childNodes[j] = new TrieNode();
            }
            p = p->childNodes[j];
        }
        p->isWordEnd = true;
    }

    int matchLength(const string &key) {
        TrieNode *p = root;
        int length = 0;
        for (int i = 0; i < key.length(); i++) {
            int j = key[i] - '0';
            if (!p->childNodes[j]) {
                return length;
            }
            p = p->childNodes[j];
            length++;
        }
        return length;
    }

    bool search(const string &key, bool prefix = false) {
        TrieNode *p = root;
        for (int i = 0; i < key.length(); i++) {
            int j = key[i] - '0';
            if (!p->childNodes[j]) {
                return false;
            }
            p = p->childNodes[j];
        }
        if (prefix == false) {
            return p->isWordEnd;
        }
        return true;
    }

    bool startsWith(const string &prefix) {
        return search(prefix, true);
    }
};

class Solution {
public:
    int longestCommonPrefix(vector<int>& arr1, vector<int>& arr2) {
        Trie* t = new Trie();
        for (const auto num1: arr1) {
            t->insert(to_string(num1));
        }
        int ans = 0;
        for (const auto num2: arr2) {
            ans = max(ans, t->matchLength(to_string(num2)));
        }
        return ans;
    }
};

/* */

#pragma GCC optimize("O3", "unroll-loops")
class TrieNode {
public:
    int data;
    TrieNode* children[10];
    bool isTerminal;

    TrieNode(int ch) {
        data = ch;
        for (int i = 0; i < 10; i++) { children[i] = NULL; }
        isTerminal = false;
    }
};

class Trie {
    TrieNode* root;
public:
    Trie() {
        root = new TrieNode(-1);
    }

    void insert(int data) {
        TrieNode* node = root;
        int divisor = 1;

        while (data / divisor >= 10) {
            divisor *= 10;
        }

        while (divisor > 0) {
            int digit = (data / divisor) % 10;
            if (node->children[digit] == NULL) {
                node->children[digit] = new TrieNode(digit);
            }
            node = node->children[digit];
            divisor /= 10;
        }
        node->isTerminal = true;
    }

    int prefixLen(int data) {
        TrieNode* node = root;
        int length = 0;
        int divisor = 1;

        while (data / divisor >= 10) {
            divisor *= 10;
        }

        while (divisor > 0) {
            int digit = (data / divisor) % 10;
            if (node->children[digit] == NULL) {
                return length;
            }
            node = node->children[digit];
            length++;
            divisor /= 10;
        }
        return length;
    }
};

class Solution {
    Trie* trie = new Trie();
public:
    int longestCommonPrefix(vector<int>& arr1, vector<int>& arr2) {
        ios::sync_with_stdio(false);
        cin.tie(0);
        cout.tie(0);

        for (const int& num : arr1) {
            trie->insert(num);
        }

        int maxLen = 0;
        for (const int& num : arr2) {
            maxLen = max(trie->prefixLen(num), maxLen);
        }

        return maxLen;
    }
};

