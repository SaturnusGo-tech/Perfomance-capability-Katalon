import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable


'LOG IN'
userLogin = WS.sendRequestAndVerify(findTestObject('Object Repository/OPUS_regression/logInFront',[
	('email') : GlobalVariable.userNameFront,
	('password') : GlobalVariable.userPasswordFront
	]))
assert WS.getElementPropertyValue(userLogin, 'errors') == [] || WS.getElementPropertyValue(userLogin, 'errors') == null  
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
def headers = response.headerFields
println headers
GlobalVariable.authorizationCookie = headers["set-cookie"][2].split(";")[0]
println GlobalVariable.authorizationCookie


'GET USER ID AND VERIFY COOKIE IS APPLIED'
userGet = WS.sendRequestAndVerify(findTestObject('Object Repository/OPUS_regression/userGet'))
assert WS.getElementPropertyValue(userGet, 'errors') == [] || WS.getElementPropertyValue(userGet, 'errors') == null
assert WS.getElementPropertyValue(userGet,'data.me.userName') != 'Anonymous'
GlobalVariable.userId = WS.getElementPropertyValue(userGet,'data.me.id')
println GlobalVariable.userId


'GET LIST OF CATEGORIES'
categoriesGet = WS.sendRequestAndVerify(findTestObject('Object Repository/OPUS_regression/childCategories',[
	('userId') : GlobalVariable.userId
	]))
assert WS.getElementPropertyValue(categoriesGet, 'errors') == [] || WS.getElementPropertyValue(categoriesGet, 'errors') == null
categories = new JsonSlurper().parseText(categoriesGet.getResponseBodyContent())
categoriesMap = categories.data.childCategories.childCategories.collectEntries { 
	category ->[(category.name): category.id]
	}
println categoriesMap


'SELECT A CATEGORY FROM THE MAP AND GET ITS ID'
GlobalVariable.categoryId = categoriesMap["Information Technology"] // input name of another category if needed
println GlobalVariable.categoryId


'GET AN AVAILABLE PRODUCT FOR EACH SUPPLIER'
productIds = []
supplierIds = [ GlobalVariable.ODP_outerId, GlobalVariable.pocketNurse_outerId ]
//start loop to get a product for each supplier
		for(int i; i < supplierIds.size(); i++) {
			
			
			'FOR SELECTED CATEGORY GET PRODUCT OFFERS'
			prooductOffersSearch = WS.sendRequestAndVerify(findTestObject('Object Repository/OPUS_regression/productOffersSearch',[
				('categoryId') : GlobalVariable.categoryId,
				('supplierOuterId') : supplierIds[i],
				('userId') : GlobalVariable.userId
				]))
			assert WS.getElementPropertyValue(prooductOffersSearch, 'errors') == [] || WS.getElementPropertyValue(prooductOffersSearch, 'errors') == null
			productOffers = new JsonSlurper().parseText(prooductOffersSearch.getResponseBodyContent()).data.products.items
			println productOffers
			
			
			'SELECT PRODUCT THAT HAS PRICE AND QUANTITY > 0'
			availableProduct = productOffers.find {
				 product -> product.availabilityData.availableQuantity > 0 && product.price.actual.amount > 0
				 }
			println availableProduct
			productId = availableProduct.id
			println productId
			productIds.add(productId)
			
		}		
GlobalVariable.ODP_productId = productIds[0]
GlobalVariable.pocketNurse_productId = productIds[1]
println productIds
println GlobalVariable.ODP_productId
println GlobalVariable.pocketNurse_productId


'ADD BOTH PRODUCTS TO CART'

	
	productAddToCart = WS.sendRequestAndVerify(findTestObject('Object Repository/OPUS_regression/productsAddToCart',[
		('userId') : GlobalVariable.userId,
		('productId') : GlobalVariable.ODP_productId
		]))
	assert WS.getElementPropertyValue(productAddToCart, 'errors') == [] || WS.getElementPropertyValue(productAddToCart, 'errors') == null	







/*
supplier2_id = '300'//Pocket Nurse 


'ADD ITEM FROM SUPPLIER 1'
'GO THROUGH CHECKOUT PROCESS'
'ADD ITEM FROM SUPPLIER 2'
'GO THROUGH CHECKOUT PROCESS'
'PLACE ORDER'
'PAY BY CARD'
'VERIFY ORDER @ORDERS PAGE'
	*/