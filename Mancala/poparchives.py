import pickle
from pathlib import Path

root = Path(".")
archives = root / "population archives"

def write(obj, filename:str):
    '''Saves the object in archives with filename.'''
    with open(archives / filename, 'wb') as output:
        pickle.dump(obj, output, pickle.HIGHEST_PROTOCOL)

def load(filename:str):
    '''Loads the object from under archives with filename.'''
    with open(archives / filename, 'rb') as input:
        return pickle.load(input)