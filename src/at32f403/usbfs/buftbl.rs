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
            .field("btaddr", &format_args!("{}", self.btaddr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BUFTBL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 3:15 - Endpoint buffer table start address"]
    #[inline(always)]
    #[must_use]
    pub fn btaddr(&mut self) -> BTADDR_W<BUFTBL_SPEC> {
        BTADDR_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Buffer table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buftbl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buftbl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFTBL_SPEC;
impl crate::RegisterSpec for BUFTBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buftbl::R`](R) reader structure"]
impl crate::Readable for BUFTBL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buftbl::W`](W) writer structure"]
impl crate::Writable for BUFTBL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFTBL to value 0"]
impl crate::Resettable for BUFTBL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
