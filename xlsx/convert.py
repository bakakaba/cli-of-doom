#! python3

import sys
import codecs
import xlsxwriter


def convertCsvToXlsx(inputFile, outputFile):
    # Open the input file with the correct encoding.
    textfile = codecs.open(inputFile, 'r', 'utf-8')

    # Create an new Excel file and convert the text data.
    workbook = xlsxwriter.Workbook(outputFile)
    worksheet = workbook.add_worksheet()

    # Start from the first cell.
    row = 0
    col = 0

    # Read the text file and write it to the worksheet.
    for line in textfile:
        # Ignore the comments in the text file.
        if line.startswith('#'):
            continue

        for column in line.split(','):
            worksheet.write(row, col, column.strip())
            col += 1

        col = 0
        row += 1

    workbook.close()


print("Input: {}".format(sys.argv[1]))
print("Output: {}".format(sys.argv[2]))
convertCsvToXlsx(sys.argv[1], sys.argv[2])
