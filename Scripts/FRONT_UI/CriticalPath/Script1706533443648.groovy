import auth.AuthificationStage
import objects.HomePage
import objects.CartPage
import objects.ShippingPage
import objects.BillingPage
import objects.Review

// Authenticate and log in
// Create an instance of the AuthenticationStage class and call the testAuth method to perform authentication and login.
AuthificationStage auth = new AuthificationStage()
auth.testAuth()

// Navigate through the home page
// Call methods from the HomePage class to check for the presence of images with specific alt texts,
// measure page load performance for the education section, click on the category and cart buttons.
HomePage.checkImagesByAltTexts()
HomePage.checkPageLoadPerformance("https://qa-opus.omniapartners.com/education", 10000)
HomePage.clickCategoryButton()
HomePage.clickCartButton()

// Navigate through the cart page and proceed to checkout
// On the CartPage, click the proceed to checkout button and check the page load performance.
CartPage.clickCheckoutProceedBtn()

// Navigate through shipping options
// On the ShippingPage, navigate through shipping options and check the page's load performance.
ShippingPage.navigateThroughShippingOptions()

// Open front features on the billing page
// On the BillingPage, open front features and check the page load performance.
BillingPage.openFrontFeatures()

// Click on 'Send to Approval' button in the review page
// On the Review page, click the 'Send to Approval' button and check the page load performance.
Review.clickSendToApprovalButton()
