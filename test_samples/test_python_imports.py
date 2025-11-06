import os
import sys
from datetime import datetime, timedelta
from collections import defaultdict
import requests
from . import local_module
from .utils import helper_function

class ImportTest:
    def __init__(self):
        self.data = defaultdict(list)

    def process_data(self):
        current_time = datetime.now()
        return current_time + timedelta(days=1)