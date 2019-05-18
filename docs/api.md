# World API

## Get a world state [GET world/{id}/get?type]
- Parameters 
    * id: `1` (string, required) - world identifier
    * type: `health` (string, optional) - type of a view to be returned (possible values are `health|sources`)
- Response 200 (application/vnd.api+json; charset=utf-8 ?)
    * data (WorldView, required) - world representation


# Data Structures

## WorldView
### Properties
* id: `1` (number)
* width: `40` (number)
* height: `20` (number)
* tick_no: `12323` (number)
* type: `health` (string) - world representation
* data: (BinaryData) - width x height bytes (could be changed in future)
* dictionary: (array[DicEntry]) 

## DicEntry
### Properties
* key: `1` (string) - how it is represented in binary data
* value: `predator` (string) - value related to this in that representation

## BinaryData
### Properties
* data: (binary:array[number]) - value of every point on a plain in a required type scaled up to 255  


