bool canArrange(int* arr, int arrSize, int k) {
    int f[k];
    memset(f,0,sizeof(f));
    for(int i=0;i<arrSize;i++){
        int cnt=arr[i]%k;
        if(cnt<0)cnt+=k;
        f[cnt]++;
    }
    if (f[0] & 1 == 1) {
        return false;
    }
    for(int i=1;i<1+k/2;i++){
        if((f[i]!=f[k-i]))return false;
    }
    // return f[0]%2?false:true;
    return true;
}

