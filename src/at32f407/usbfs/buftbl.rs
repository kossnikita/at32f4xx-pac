#[doc = "Register `BUFTBL` reader"]
pub type R = crate::R<BUFTBL_SPEC>;
#[doc = "Register `BUFTBL` writer"]
pub type W = crate::W<BUFTBL_SPEC>;
#[doc = "Field `BTADDR` reader - Endpoint buffer table start address"]
pub type BTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `BTADDR` writer - Endpoint buffer table start address"]
pub type BTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - Endpoint buffer table start address"]
    #[inline(always)]
    pub fn btaddr(&self) -> BTADDR_R {
        BTADDR_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUFTBL")
            .field("btaddr", &self.btaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:15 - Endpoint buffer table start address"]
    #[inline(always)]
    pub fn btaddr(&mut self) -> BTADDR_W<'_, BUFTBL_SPEC> {
        BTADDR_W::new(self, 3)
    }
}
#[doc = "Buffer table address\n\nYou can [`read`](crate::Reg::read) this register and get [`buftbl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buftbl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFTBL_SPEC;
impl crate::RegisterSpec for BUFTBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buftbl::R`](R) reader structure"]
impl crate::Readable for BUFTBL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buftbl::W`](W) writer structure"]
impl crate::Writable for BUFTBL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFTBL to value 0"]
impl crate::Resettable for BUFTBL_SPEC {}
