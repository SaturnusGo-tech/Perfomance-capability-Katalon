import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

WebUI.comment('TEST CASE: setup supplier portal backend')

GlobalVariable.urlBack = GlobalVariable.supplierPortalUrl


'CREATE KATALON OPERATOR'
WS.callTestCase(findTestCase('Test Cases/KatalonSampleDataOperations/KatalonOperatorCreate'), null)


'CREATE MASTER CATALOG'//hardcoded put request
masterCatalogCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.Catalog/masterCatalogCreate'))



'CREATE MAIN CATEGORY'
GlobalVariable.categoryName = 'main category'
categoryCreate = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Catalog/CategoriesCreate', [
	('catalogId') : GlobalVariable.supplierPortalMasterCatalogId,
	('code') : 'MFD',
	('name') : 'main category'
	]))
//WS.verifyElementPropertyValue(categoryCreate, 'catalogId', GlobalVariable.catalogId)
//WS.verifyElementPropertyValue(categoryCreate, 'name', GlobalVariable.categoryName)
//WS.verifyElementPropertyValue(categoryCreate, 'code', GlobalVariable.categoryCode)


'CREATE CURRENCY'
currencyCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Core/currencyCreate',[
	('currencyName') : 'USD',
	('currencyCode') : 'USD'
	]))


'CREATE MASTER STORE (MFD) WITH ALL THE REQUIRED SETTINGS'//hardcoded to avoid long and cumbersome process of create update
masterStoreCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.Store/masterStoreCreateVSP'))


'PART 2 // Actual tests // CREATE VENDOR'


'CREATE VENDOR'
WS.callTestCase(findTestCase('Test Cases/API Coverage/_OPUS/_OPUS_SupplierPortal/vendorCreateUpdate'),null)
WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleSearch/BuildIndex'), null)
WS.callTestCase(findTestCase('Test Cases/API Coverage/_OPUS/_OPUS_SupplierPortal/vendorLogin'),null)


'PART 3'
WebUI.comment('TEST CASE: CREATE IMPORT PROFILE AND IMPORT PRODUCTS')


'CREATE IMPORT PROFILE'
profileCreate = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/importProfileAdd',[
	('name') : GlobalVariable.katalonImportProfileName ,
	('userId') : GlobalVariable.katalonVendorUserId,
	('username') : GlobalVariable.katalonVendorEmail
	]))


'VERIFY THE PROFILE HAS BEEN CREATED AND GET ITS ID'
profileVerifyCreated = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/importProfileSearch',[
	('keyword') : GlobalVariable.katalonImportProfileName 
	]))
WS.verifyElementPropertyValue(profileVerifyCreated,'results[0].name' , GlobalVariable.katalonImportProfileName )
GlobalVariable.katalonImportProfileId = WS.getElementPropertyValue(profileVerifyCreated, 'results[0].id')



//PART 4
'CATALOG UPLOAD'
'UPLOAD PRODUCTS CSV TO THE TEMPORARY FOLDER'
GlobalVariable.uploadedFileName = 'supProducts_js-05_1.csv'
GlobalVariable.folderUrl = '/tmp'
uploadFileUrlLocal = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.Assets/vspProducts_AssetFileUploadLocal', [
	('folderUrl') : GlobalVariable.folderUrl,
	('fileName') : GlobalVariable.uploadedFileName
	]))
//get file url
GlobalVariable.uploadFileUrl = WS.getElementPropertyValue(uploadFileUrlLocal, '[0].url')
println GlobalVariable.uploadFileUrl
//return uploadFileUrlLocal


'GET VENDOR TOKEN TO AUTHORIZE THE REQUEST'
GlobalVariable.adminPassword = 'Password1!'
def login = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/Authorization/AuthorizationTokenNew',[
	('username') : GlobalVariable.katalonVendorEmail,
	('password') : GlobalVariable.adminPassword
	]))
GlobalVariable.token = WS.getElementPropertyValue(login, 'token_type') + ' ' + WS.getElementPropertyValue(login, 'access_token')
println GlobalVariable.token


'IMPORT UPLOADED CSV'

//GlobalVariable.katalonVendorName = 'katalonVendor'
//GlobalVariable.katalonVendorId = 'cefbafe2-5f1b-47be-9de2-8d77df3c3d8e'
//for some reason can only be authorized via token, no  apiKey auth. Bug mb?
fileImport = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/VirtoCommerce.marketplaceVendorExtension/importRunFileCsv',[
	('name') : GlobalVariable.katalonImportProfileName ,
	('id') : GlobalVariable.katalonVendorId,
	('userName') : GlobalVariable.katalonVendorName,
	('objectId') : GlobalVariable.katalonImportProfileId,
	('importFileUrl') : GlobalVariable.uploadFileUrl
	]))




