int main()
{
    int x = (int)'F' << 24 | (int)'E' << 16 | (int)'L' << 8 | 0x7f;
    printf("%x\n", x);
    return 0;
}