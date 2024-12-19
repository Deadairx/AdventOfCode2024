import argparse

def extract_columns_from_file(file_path):
    """Read the contents of a file and return two arrays, each containing a column of numbers."""
    column1 = []
    column2 = []
    with open(file_path, 'r') as file:
        for line in file:
            numbers = line.split()
            if len(numbers) >= 2:
                column1.append(float(numbers[0]))
                column2.append(float(numbers[1]))
    return column1, column2

def main():
    parser = argparse.ArgumentParser(description='Read a file and print its contents.')
    parser.add_argument('file_path', type=str, help='The path to the file to be read')
    args = parser.parse_args()

    column1, column2 = extract_columns_from_file(args.file_path)
    column1.sort()
    column2.sort()
    sum = 0
    for i in range(len(column1)):
        count = 0
        for j in range(len(column2)):
            if column1[i] == column2[j]:
                count += 1

        sum += column1[i] * count

    print(sum)

if __name__ == '__main__':
    main()

