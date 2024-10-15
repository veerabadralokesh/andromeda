long long minimumSteps(char* s) {
    long long ans = 0;
    long long ones = 0;
    for (int i=0; s[i] != '\0'; i++) {
        // if (s[i] == '1') {
        //     ones++;
        // } else {
        //     ans += ones;
        // }
        switch (s[i]) {
		case '0':
			ans += ones;
			break;
		case '1':
			ones++;
			break;
		default:
			assert(0);
        }
    }
    return ans;
}

