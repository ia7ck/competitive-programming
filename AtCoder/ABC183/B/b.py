def main():
    sx, sy, gx, gy = map(int, input().split())

    print("{:.10f}".format(sy * (gx - sx) / (gy + sy) + sx))


if __name__ == "__main__":
    main()
