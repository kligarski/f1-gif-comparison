import fastf1, sys

# LAP_STATS = [
#     "Sector1Time", 
#     "Sector2Time", 
#     "Sector3Time", 
#     "Sector1SessionTime", 
#     "Sector2SessionTime", 
#     "Sector3SessionTime"]

# Format: python fetch.py <year> <country?> <driver1> <driver2> 
year = int(sys.argv[1])
country = sys.argv[2]
driver1 = sys.argv[3]
driver2 = sys.argv[4]

session = fastf1.get_session(year, country, 'Q')
session.load()

driver1_lap = session.laps.pick_driver(driver1).pick_fastest()
driver2_lap = session.laps.pick_driver(driver2).pick_fastest()

# driver1_lap_stats = driver1_lap[LAP_STATS]
# driver2_lap_stats = driver2_lap[LAP_STATS]

# driver1_stats = session.get_driver(driver1)
# driver2_stats = session.get_driver(driver2)

driver1_telemetry = driver1_lap.get_telemetry(frequency=20)
driver2_telemetry = driver2_lap.get_telemetry(frequency=20)

driver1_telemetry.X = driver1_telemetry.X.map(lambda x: int(x)) 
driver1_telemetry.Y = driver1_telemetry.Y.map(lambda x: int(x)) 

driver2_telemetry.X = driver2_telemetry.X.map(lambda x: int(x)) 
driver2_telemetry.Y = driver2_telemetry.Y.map(lambda x: int(x)) 

with open("driver1_telemetry.json", "w") as file:
    driver1_telemetry[['X', 'Y']].rename(columns={"X": "x", "Y": "y"}).to_json(file, orient="records")

with open("driver2_telemetry.json", "w") as file:
    driver2_telemetry[['X', 'Y']].rename(columns={"X": "x", "Y": "y"}).to_json(file, orient="records")