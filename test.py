from pybinwalk import scan, extract

path = "/home/purge/Downloads/Team-battery.jpg"
output_path = "/home/purge/Downloads/extracted_output_121"
a = scan(path)
b = extract(path, output_path)
print(a)
print(b)
