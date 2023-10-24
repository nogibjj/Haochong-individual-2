import pandas as pd
import time
from mylib.lib import get_median
import psutil

# Record the start time
start_time = time.time()

if __name__ == "__main__":
    example_csv = "25ktopomapseriesindex.csv"
    df = pd.read_csv(example_csv)
    print(get_median(df))
    end_time = time.time()

# Calculate the elapsed time
elapsed_time = end_time - start_time

cpu_percent = psutil.cpu_percent()
memory_info = psutil.virtual_memory()

print(f"Elapsed time: {elapsed_time:.4f} seconds")
print(f"CPU Usage: {cpu_percent}%")
print(f"Memory Usage: {memory_info.percent}%")
