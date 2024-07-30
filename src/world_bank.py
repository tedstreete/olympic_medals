import requests
import pandas as pd

# Define the World Bank API URLs for GDP and Population data
gdp_url = "http://api.worldbank.org/v2/country/all/indicator/NY.GDP.MKTP.CD?format=json&per_page=20000"
pop_url = "http://api.worldbank.org/v2/country/all/indicator/SP.POP.TOTL?format=json&per_page=20000"

# Fetch GDP data
gdp_response = requests.get(gdp_url)
gdp_data = gdp_response.json()[1]

# Fetch Population data
pop_response = requests.get(pop_url)
pop_data = pop_response.json()[1]

# Create DataFrames
gdp_df = pd.DataFrame(gdp_data)[['countryiso3code', 'country', 'value']]
gdp_df.columns = ['country_code', 'country_name', 'gdp']

pop_df = pd.DataFrame(pop_data)[['countryiso3code', 'country', 'value']]
pop_df.columns = ['country_code', 'country_name', 'population']

# Merge DataFrames on country_code
merged_df = pd.merge(gdp_df, pop_df, on=['country_code', 'country_name'])

# Drop rows with NaN values (if any)
merged_df.dropna(inplace=True)

# Write to CSV
merged_df.to_csv('gdp_population.csv', index=False)
