#!/bin/env python3


import json
import requests

# Définir l'URL de l'API REST
url = "http://192.168.122.100:6777/data"

# Ouvrir le fichier en lecture
with open("data.csv", "r") as fichier:
    # Lire le contenu du fichier
    contenu = fichier.read()

# Séparer le contenu du fichier en lignes
lignes = contenu.split("\n")

# Boucler sur chaque ligne et envoyer les données à l'API REST
for ligne in lignes:
    if ligne:
        # Séparer les champs de la ligne
        champs = ligne.split(";")
        # Créer un dictionnaire pour stocker les données de la ligne
        donnees_ligne = {
            "date": champs[0],
            "time": champs[1],
            "global_active_power": float(champs[2]),
            "global_reactive_power": float(champs[3]),
            "voltage": float(champs[4]),
            "global_intensity": float(champs[5]),
            "sub_metering_1": float(champs[6]),
            "sub_metering_2": float(champs[7]),
            "sub_metering_4": float(champs[8])
        }
        # Convertir les données en JSON
        donnees_json = json.dumps(donnees_ligne)
        # Envoyer les données à l'API REST en utilisant la méthode POST
        response = requests.post(url, json=donnees_json)
        if response.ok:
            print("Données envoyées avec succès à l'API REST : ", donnees_ligne)
        else:
            print("Erreur lors de l'envoi des données à l'API REST : ", response.text)

