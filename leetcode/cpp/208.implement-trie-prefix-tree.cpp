using namespace std;

struct TrieNode {
    vector<shared_ptr<TrieNode>> children;
    bool isEnd = false;
    TrieNode() : children(26) {}
};

class Trie {
public:
    Trie() { root = make_shared<TrieNode>(); }

    void insert(string word) {
        shared_ptr<TrieNode> node = root;
        for (const char c : word) {
            const int i = c - 'a';
            if (node->children[i] == nullptr) {
                node->children[i] = make_shared<TrieNode>();
            }
            node = node->children[i];
        }
        node->isEnd = true;
    }

    bool search(string word) {
        shared_ptr<TrieNode> node = find(word);
        return node && node->isEnd;
    }

    bool startsWith(string prefix) { return find(prefix) != nullptr; }

private:
    shared_ptr<TrieNode> root;
    shared_ptr<TrieNode> find(const string& prefix) {
        shared_ptr<TrieNode> node = root;
        for (const char c : prefix) {
            const int i = c - 'a';
            if (node->children[i] == nullptr) {
                return nullptr;
            }
            node = node->children[i];
        }
        return node;
    }
};

/**
 * Your Trie object will be instantiated and called as such:
 * Trie* obj = new Trie();
 * obj->insert(word);
 * bool param_2 = obj->search(word);
 * bool param_3 = obj->startsWith(prefix);
 */

/* */

#define ALPHABETS 26

class TrieNode {
   public:
    TrieNode *childNodes[ALPHABETS];
    bool isWordEnd;
    TrieNode() {
        isWordEnd = false;
        for (int i = 0; i < ALPHABETS; i++) {
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
            int j = key[i] - 'a';
            if (!p->childNodes[j]) {
                p->childNodes[j] = new TrieNode();
            }
            p = p->childNodes[j];
        }
        p->isWordEnd = true;
    }

    bool search(const string &key, bool prefix = false) {
        TrieNode *p = root;
        for (int i = 0; i < key.length(); i++) {
            int j = key[i] - 'a';
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

