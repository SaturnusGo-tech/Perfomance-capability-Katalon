import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable


WebUI.comment('TEST CASE: CREATE AND VERIFY CHANGES IN THE CHANGES LOG')


'GET KATALON CATALOG ID TO CREATE THE CATALOG THERE'
katalonCatalgSearch = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleCatalog/catalogSearch'),
	null)



'IN THE CATALOG CREATE A CATEGORY'
categoryCreate = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleCatalog/categoryCreate'),
	null)
//'SEARCH FOR CATEGORY AND GET THE ID TO CREATE THE CATALOG THERE'
//catalogCategoriesSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Catalog/CategoriesSearch',[
//	('catalogId') : GlobalVariable.catalogId
//	]))
//GlobalVariable.categoryId = WS.getElementPropertyValue(catalogCategoriesSearch,'items[0].id')
//
//
//'IN THE CATALOG CREATE A PRODUCT'
//productCreate = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleCatalog/productCreate'),
//	null)


'EXTRACT "OBJECT TYPE OF THE CREATED CATEGORY'
objectType = WS.getElementPropertyValue(categoryCreate, 'seoObjectType')
println objectType


'EXTRACT CREATED CATEGORY CREATED/MODIFIED DATES AND VERIFY THEY MATCH'
productCreatedDate = WS.getElementPropertyValue(categoryCreate, 'createdDate')
productModifiedDate = WS.getElementPropertyValue(categoryCreate, 'modifiedDate')
assert productCreatedDate == productModifiedDate
println productModifiedDate


'SEARCH FOR THE CHANGES LOG AND VERIFY CREATED CHANGES ARE LOGGED AND RELATE'
operationType = 'Added'
searchChangeLog = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/ChangelogSearch'))
WS.verifyElementPropertyValue(searchChangeLog, '[0].objectType', objectType)
WS.verifyElementPropertyValue(searchChangeLog, '[0].objectId', GlobalVariable.categoryId)
WS.verifyElementPropertyValue(searchChangeLog, '[0].operationType', operationType)


'GET THE CHANGES LOG AND VERIFY CREATED CHANGES ARE LOGGED AND RELATE'
getChangeLog = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/ChangelogGetChanges', [
	('type') : objectType
	]))
WS.verifyElementPropertyValue(searchChangeLog, '[0].objectType', objectType)
WS.verifyElementPropertyValue(searchChangeLog, '[0].objectId', GlobalVariable.categoryId)
WS.verifyElementPropertyValue(searchChangeLog, '[0].operationType', operationType)


'DELETE THE CREATED CATEGORY'
//category can not be deleted in the OMNIA configuration, so 
//it will just stay there for now
//categoryDelete = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModuleCatalog/categoryDelete'), 
//	null)

