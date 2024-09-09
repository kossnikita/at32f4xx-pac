#[doc = "Register `ODT` reader"]
pub type R = crate::R<ODT_SPEC>;
#[doc = "Field `ODT` reader - Conversion data of ordinary channel"]
pub type ODT_R = crate::FieldReader<u16>;
#[doc = "Field `ADC2ODT` reader - ADC2 conversion data of ordinary channel"]
pub type ADC2ODT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Conversion data of ordinary channel"]
    #[inline(always)]
    pub fn odt(&self) -> ODT_R {
        ODT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADC2 conversion data of ordinary channel"]
    #[inline(always)]
    pub fn adc2odt(&self) -> ADC2ODT_R {
        ADC2ODT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODT")
            .field("adc2odt", &self.adc2odt())
            .field("odt", &self.odt())
            .finish()
    }
}
#[doc = "Ordinary data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODT_SPEC;
impl crate::RegisterSpec for ODT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odt::R`](R) reader structure"]
impl crate::Readable for ODT_SPEC {}
#[doc = "`reset()` method sets ODT to value 0"]
impl crate::Resettable for ODT_SPEC {
    const RESET_VALUE: u32 = 0;
}
