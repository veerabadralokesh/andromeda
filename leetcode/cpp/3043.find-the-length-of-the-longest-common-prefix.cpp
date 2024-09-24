
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

