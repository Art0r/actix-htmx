import json
import random

import requests
from faker import Faker

def populate_users(faker: Faker) -> list[dict]:

    users = []
    for i in range(10):

        name = faker.name()
        email = faker.email()
        password = "123"

        data = {
            "name": name,
            "email": email,
            "password": password
        }

        res = requests.post("http://localhost:8080/api/users", json=data)
        users.append(json.loads(res.content))

    return users

def populate_pets(faker: Faker, users: list[dict]) -> list[dict]:

    pets = []
    for i in range(10):

        name = faker.name()
        description = faker.text(500)
        tutor = random.choice(users)
        tutor_id = tutor.get('id')

        data = {
            "name": name,
            "description": description,
            "tutor_id": tutor_id
        }

        res = requests.post("http://localhost:8080/api/pets", json=data)
        pets.append(json.loads(res.content))

    return pets

def main():

    faker = Faker()

    users = populate_users(faker)
    pets = populate_pets(faker, users)


if __name__ == "__main__":
    main()